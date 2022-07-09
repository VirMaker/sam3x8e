#[doc = "Register `MAM7` reader"]
pub struct R(crate::R<MAM7_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MAM7_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MAM7_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MAM7_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MAM7` writer"]
pub struct W(crate::W<MAM7_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MAM7_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<MAM7_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MAM7_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MIDvB` reader - Complementary bits for identifier in extended frame mode"]
pub type MIDVB_R = crate::FieldReader<u32, u32>;
#[doc = "Field `MIDvB` writer - Complementary bits for identifier in extended frame mode"]
pub type MIDVB_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MAM7_SPEC, u32, u32, 18, O>;
#[doc = "Field `MIDvA` reader - Identifier for standard frame mode"]
pub type MIDVA_R = crate::FieldReader<u16, u16>;
#[doc = "Field `MIDvA` writer - Identifier for standard frame mode"]
pub type MIDVA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MAM7_SPEC, u16, u16, 11, O>;
#[doc = "Field `MIDE` reader - Identifier Version"]
pub type MIDE_R = crate::BitReader<bool>;
#[doc = "Field `MIDE` writer - Identifier Version"]
pub type MIDE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MAM7_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:17 - Complementary bits for identifier in extended frame mode"]
    #[inline(always)]
    pub fn midv_b(&self) -> MIDVB_R {
        MIDVB_R::new((self.bits & 0x0003_ffff) as u32)
    }
    #[doc = "Bits 18:28 - Identifier for standard frame mode"]
    #[inline(always)]
    pub fn midv_a(&self) -> MIDVA_R {
        MIDVA_R::new(((self.bits >> 18) & 0x07ff) as u16)
    }
    #[doc = "Bit 29 - Identifier Version"]
    #[inline(always)]
    pub fn mide(&self) -> MIDE_R {
        MIDE_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:17 - Complementary bits for identifier in extended frame mode"]
    #[inline(always)]
    pub fn midv_b(&mut self) -> MIDVB_W<0> {
        MIDVB_W::new(self)
    }
    #[doc = "Bits 18:28 - Identifier for standard frame mode"]
    #[inline(always)]
    pub fn midv_a(&mut self) -> MIDVA_W<18> {
        MIDVA_W::new(self)
    }
    #[doc = "Bit 29 - Identifier Version"]
    #[inline(always)]
    pub fn mide(&mut self) -> MIDE_W<29> {
        MIDE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Mailbox Acceptance Mask Register (MB = 7)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mam7](index.html) module"]
pub struct MAM7_SPEC;
impl crate::RegisterSpec for MAM7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mam7::R](R) reader structure"]
impl crate::Readable for MAM7_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mam7::W](W) writer structure"]
impl crate::Writable for MAM7_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MAM7 to value 0"]
impl crate::Resettable for MAM7_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
