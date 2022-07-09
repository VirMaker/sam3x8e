#[doc = "Register `EBCIDR` writer"]
pub struct W(crate::W<EBCIDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EBCIDR_SPEC>;
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
impl From<crate::W<EBCIDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EBCIDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BTC0` writer - Buffer Transfer Completed \\[5:0\\]"]
pub type BTC0_W<'a, const O: u8> = crate::BitWriter<'a, u32, EBCIDR_SPEC, bool, O>;
#[doc = "Field `BTC1` writer - Buffer Transfer Completed \\[5:0\\]"]
pub type BTC1_W<'a, const O: u8> = crate::BitWriter<'a, u32, EBCIDR_SPEC, bool, O>;
#[doc = "Field `BTC2` writer - Buffer Transfer Completed \\[5:0\\]"]
pub type BTC2_W<'a, const O: u8> = crate::BitWriter<'a, u32, EBCIDR_SPEC, bool, O>;
#[doc = "Field `BTC3` writer - Buffer Transfer Completed \\[5:0\\]"]
pub type BTC3_W<'a, const O: u8> = crate::BitWriter<'a, u32, EBCIDR_SPEC, bool, O>;
#[doc = "Field `BTC4` writer - Buffer Transfer Completed \\[5:0\\]"]
pub type BTC4_W<'a, const O: u8> = crate::BitWriter<'a, u32, EBCIDR_SPEC, bool, O>;
#[doc = "Field `BTC5` writer - Buffer Transfer Completed \\[5:0\\]"]
pub type BTC5_W<'a, const O: u8> = crate::BitWriter<'a, u32, EBCIDR_SPEC, bool, O>;
#[doc = "Field `CBTC0` writer - Chained Buffer Transfer Completed \\[5:0\\]"]
pub type CBTC0_W<'a, const O: u8> = crate::BitWriter<'a, u32, EBCIDR_SPEC, bool, O>;
#[doc = "Field `CBTC1` writer - Chained Buffer Transfer Completed \\[5:0\\]"]
pub type CBTC1_W<'a, const O: u8> = crate::BitWriter<'a, u32, EBCIDR_SPEC, bool, O>;
#[doc = "Field `CBTC2` writer - Chained Buffer Transfer Completed \\[5:0\\]"]
pub type CBTC2_W<'a, const O: u8> = crate::BitWriter<'a, u32, EBCIDR_SPEC, bool, O>;
#[doc = "Field `CBTC3` writer - Chained Buffer Transfer Completed \\[5:0\\]"]
pub type CBTC3_W<'a, const O: u8> = crate::BitWriter<'a, u32, EBCIDR_SPEC, bool, O>;
#[doc = "Field `CBTC4` writer - Chained Buffer Transfer Completed \\[5:0\\]"]
pub type CBTC4_W<'a, const O: u8> = crate::BitWriter<'a, u32, EBCIDR_SPEC, bool, O>;
#[doc = "Field `CBTC5` writer - Chained Buffer Transfer Completed \\[5:0\\]"]
pub type CBTC5_W<'a, const O: u8> = crate::BitWriter<'a, u32, EBCIDR_SPEC, bool, O>;
#[doc = "Field `ERR0` writer - Access Error \\[5:0\\]"]
pub type ERR0_W<'a, const O: u8> = crate::BitWriter<'a, u32, EBCIDR_SPEC, bool, O>;
#[doc = "Field `ERR1` writer - Access Error \\[5:0\\]"]
pub type ERR1_W<'a, const O: u8> = crate::BitWriter<'a, u32, EBCIDR_SPEC, bool, O>;
#[doc = "Field `ERR2` writer - Access Error \\[5:0\\]"]
pub type ERR2_W<'a, const O: u8> = crate::BitWriter<'a, u32, EBCIDR_SPEC, bool, O>;
#[doc = "Field `ERR3` writer - Access Error \\[5:0\\]"]
pub type ERR3_W<'a, const O: u8> = crate::BitWriter<'a, u32, EBCIDR_SPEC, bool, O>;
#[doc = "Field `ERR4` writer - Access Error \\[5:0\\]"]
pub type ERR4_W<'a, const O: u8> = crate::BitWriter<'a, u32, EBCIDR_SPEC, bool, O>;
#[doc = "Field `ERR5` writer - Access Error \\[5:0\\]"]
pub type ERR5_W<'a, const O: u8> = crate::BitWriter<'a, u32, EBCIDR_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - Buffer Transfer Completed \\[5:0\\]"]
    #[inline(always)]
    pub fn btc0(&mut self) -> BTC0_W<0> {
        BTC0_W::new(self)
    }
    #[doc = "Bit 1 - Buffer Transfer Completed \\[5:0\\]"]
    #[inline(always)]
    pub fn btc1(&mut self) -> BTC1_W<1> {
        BTC1_W::new(self)
    }
    #[doc = "Bit 2 - Buffer Transfer Completed \\[5:0\\]"]
    #[inline(always)]
    pub fn btc2(&mut self) -> BTC2_W<2> {
        BTC2_W::new(self)
    }
    #[doc = "Bit 3 - Buffer Transfer Completed \\[5:0\\]"]
    #[inline(always)]
    pub fn btc3(&mut self) -> BTC3_W<3> {
        BTC3_W::new(self)
    }
    #[doc = "Bit 4 - Buffer Transfer Completed \\[5:0\\]"]
    #[inline(always)]
    pub fn btc4(&mut self) -> BTC4_W<4> {
        BTC4_W::new(self)
    }
    #[doc = "Bit 5 - Buffer Transfer Completed \\[5:0\\]"]
    #[inline(always)]
    pub fn btc5(&mut self) -> BTC5_W<5> {
        BTC5_W::new(self)
    }
    #[doc = "Bit 8 - Chained Buffer Transfer Completed \\[5:0\\]"]
    #[inline(always)]
    pub fn cbtc0(&mut self) -> CBTC0_W<8> {
        CBTC0_W::new(self)
    }
    #[doc = "Bit 9 - Chained Buffer Transfer Completed \\[5:0\\]"]
    #[inline(always)]
    pub fn cbtc1(&mut self) -> CBTC1_W<9> {
        CBTC1_W::new(self)
    }
    #[doc = "Bit 10 - Chained Buffer Transfer Completed \\[5:0\\]"]
    #[inline(always)]
    pub fn cbtc2(&mut self) -> CBTC2_W<10> {
        CBTC2_W::new(self)
    }
    #[doc = "Bit 11 - Chained Buffer Transfer Completed \\[5:0\\]"]
    #[inline(always)]
    pub fn cbtc3(&mut self) -> CBTC3_W<11> {
        CBTC3_W::new(self)
    }
    #[doc = "Bit 12 - Chained Buffer Transfer Completed \\[5:0\\]"]
    #[inline(always)]
    pub fn cbtc4(&mut self) -> CBTC4_W<12> {
        CBTC4_W::new(self)
    }
    #[doc = "Bit 13 - Chained Buffer Transfer Completed \\[5:0\\]"]
    #[inline(always)]
    pub fn cbtc5(&mut self) -> CBTC5_W<13> {
        CBTC5_W::new(self)
    }
    #[doc = "Bit 16 - Access Error \\[5:0\\]"]
    #[inline(always)]
    pub fn err0(&mut self) -> ERR0_W<16> {
        ERR0_W::new(self)
    }
    #[doc = "Bit 17 - Access Error \\[5:0\\]"]
    #[inline(always)]
    pub fn err1(&mut self) -> ERR1_W<17> {
        ERR1_W::new(self)
    }
    #[doc = "Bit 18 - Access Error \\[5:0\\]"]
    #[inline(always)]
    pub fn err2(&mut self) -> ERR2_W<18> {
        ERR2_W::new(self)
    }
    #[doc = "Bit 19 - Access Error \\[5:0\\]"]
    #[inline(always)]
    pub fn err3(&mut self) -> ERR3_W<19> {
        ERR3_W::new(self)
    }
    #[doc = "Bit 20 - Access Error \\[5:0\\]"]
    #[inline(always)]
    pub fn err4(&mut self) -> ERR4_W<20> {
        ERR4_W::new(self)
    }
    #[doc = "Bit 21 - Access Error \\[5:0\\]"]
    #[inline(always)]
    pub fn err5(&mut self) -> ERR5_W<21> {
        ERR5_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMAC Error, Chained Buffer Transfer Completed Interrupt and Buffer Transfer Completed Interrupt Disable register.\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ebcidr](index.html) module"]
pub struct EBCIDR_SPEC;
impl crate::RegisterSpec for EBCIDR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [ebcidr::W](W) writer structure"]
impl crate::Writable for EBCIDR_SPEC {
    type Writer = W;
}
