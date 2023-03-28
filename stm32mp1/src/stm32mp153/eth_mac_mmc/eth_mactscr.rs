#[doc = "Register `ETH_MACTSCR` reader"]
pub struct R(crate::R<ETH_MACTSCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETH_MACTSCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETH_MACTSCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETH_MACTSCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ETH_MACTSCR` writer"]
pub struct W(crate::W<ETH_MACTSCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ETH_MACTSCR_SPEC>;
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
impl From<crate::W<ETH_MACTSCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ETH_MACTSCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TSENA` reader - TSENA"]
pub type TSENA_R = crate::BitReader<bool>;
#[doc = "Field `TSENA` writer - TSENA"]
pub type TSENA_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_MACTSCR_SPEC, bool, O>;
#[doc = "Field `TSCFUPDT` reader - TSCFUPDT"]
pub type TSCFUPDT_R = crate::BitReader<bool>;
#[doc = "Field `TSCFUPDT` writer - TSCFUPDT"]
pub type TSCFUPDT_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_MACTSCR_SPEC, bool, O>;
#[doc = "Field `TSINIT` reader - TSINIT"]
pub type TSINIT_R = crate::BitReader<bool>;
#[doc = "Field `TSINIT` writer - TSINIT"]
pub type TSINIT_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_MACTSCR_SPEC, bool, O>;
#[doc = "Field `TSUPDT` reader - TSUPDT"]
pub type TSUPDT_R = crate::BitReader<bool>;
#[doc = "Field `TSUPDT` writer - TSUPDT"]
pub type TSUPDT_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_MACTSCR_SPEC, bool, O>;
#[doc = "Field `TSADDREG` reader - TSADDREG"]
pub type TSADDREG_R = crate::BitReader<bool>;
#[doc = "Field `TSADDREG` writer - TSADDREG"]
pub type TSADDREG_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_MACTSCR_SPEC, bool, O>;
#[doc = "Field `TSENALL` reader - TSENALL"]
pub type TSENALL_R = crate::BitReader<bool>;
#[doc = "Field `TSENALL` writer - TSENALL"]
pub type TSENALL_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_MACTSCR_SPEC, bool, O>;
#[doc = "Field `TSCTRLSSR` reader - TSCTRLSSR"]
pub type TSCTRLSSR_R = crate::BitReader<bool>;
#[doc = "Field `TSCTRLSSR` writer - TSCTRLSSR"]
pub type TSCTRLSSR_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_MACTSCR_SPEC, bool, O>;
#[doc = "Field `TSVER2ENA` reader - TSVER2ENA"]
pub type TSVER2ENA_R = crate::BitReader<bool>;
#[doc = "Field `TSVER2ENA` writer - TSVER2ENA"]
pub type TSVER2ENA_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_MACTSCR_SPEC, bool, O>;
#[doc = "Field `TSIPENA` reader - TSIPENA"]
pub type TSIPENA_R = crate::BitReader<bool>;
#[doc = "Field `TSIPENA` writer - TSIPENA"]
pub type TSIPENA_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_MACTSCR_SPEC, bool, O>;
#[doc = "Field `TSIPV6ENA` reader - TSIPV6ENA"]
pub type TSIPV6ENA_R = crate::BitReader<bool>;
#[doc = "Field `TSIPV6ENA` writer - TSIPV6ENA"]
pub type TSIPV6ENA_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_MACTSCR_SPEC, bool, O>;
#[doc = "Field `TSIPV4ENA` reader - TSIPV4ENA"]
pub type TSIPV4ENA_R = crate::BitReader<bool>;
#[doc = "Field `TSIPV4ENA` writer - TSIPV4ENA"]
pub type TSIPV4ENA_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_MACTSCR_SPEC, bool, O>;
#[doc = "Field `TSEVNTENA` reader - TSEVNTENA"]
pub type TSEVNTENA_R = crate::BitReader<bool>;
#[doc = "Field `TSEVNTENA` writer - TSEVNTENA"]
pub type TSEVNTENA_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_MACTSCR_SPEC, bool, O>;
#[doc = "Field `TSMSTRENA` reader - TSMSTRENA"]
pub type TSMSTRENA_R = crate::BitReader<bool>;
#[doc = "Field `TSMSTRENA` writer - TSMSTRENA"]
pub type TSMSTRENA_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_MACTSCR_SPEC, bool, O>;
#[doc = "Field `SNAPTYPSEL` reader - SNAPTYPSEL"]
pub type SNAPTYPSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SNAPTYPSEL` writer - SNAPTYPSEL"]
pub type SNAPTYPSEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ETH_MACTSCR_SPEC, u8, u8, 2, O>;
#[doc = "Field `TSENMACADDR` reader - TSENMACADDR"]
pub type TSENMACADDR_R = crate::BitReader<bool>;
#[doc = "Field `TSENMACADDR` writer - TSENMACADDR"]
pub type TSENMACADDR_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_MACTSCR_SPEC, bool, O>;
#[doc = "Field `CSC` reader - CSC"]
pub type CSC_R = crate::BitReader<bool>;
#[doc = "Field `TXTSSTSM` reader - TXTSSTSM"]
pub type TXTSSTSM_R = crate::BitReader<bool>;
#[doc = "Field `TXTSSTSM` writer - TXTSSTSM"]
pub type TXTSSTSM_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_MACTSCR_SPEC, bool, O>;
#[doc = "Field `AV8021ASMEN` reader - AV8021ASMEN"]
pub type AV8021ASMEN_R = crate::BitReader<bool>;
#[doc = "Field `AV8021ASMEN` writer - AV8021ASMEN"]
pub type AV8021ASMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_MACTSCR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - TSENA"]
    #[inline(always)]
    pub fn tsena(&self) -> TSENA_R {
        TSENA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TSCFUPDT"]
    #[inline(always)]
    pub fn tscfupdt(&self) -> TSCFUPDT_R {
        TSCFUPDT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TSINIT"]
    #[inline(always)]
    pub fn tsinit(&self) -> TSINIT_R {
        TSINIT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TSUPDT"]
    #[inline(always)]
    pub fn tsupdt(&self) -> TSUPDT_R {
        TSUPDT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - TSADDREG"]
    #[inline(always)]
    pub fn tsaddreg(&self) -> TSADDREG_R {
        TSADDREG_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - TSENALL"]
    #[inline(always)]
    pub fn tsenall(&self) -> TSENALL_R {
        TSENALL_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - TSCTRLSSR"]
    #[inline(always)]
    pub fn tsctrlssr(&self) -> TSCTRLSSR_R {
        TSCTRLSSR_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - TSVER2ENA"]
    #[inline(always)]
    pub fn tsver2ena(&self) -> TSVER2ENA_R {
        TSVER2ENA_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - TSIPENA"]
    #[inline(always)]
    pub fn tsipena(&self) -> TSIPENA_R {
        TSIPENA_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - TSIPV6ENA"]
    #[inline(always)]
    pub fn tsipv6ena(&self) -> TSIPV6ENA_R {
        TSIPV6ENA_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - TSIPV4ENA"]
    #[inline(always)]
    pub fn tsipv4ena(&self) -> TSIPV4ENA_R {
        TSIPV4ENA_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - TSEVNTENA"]
    #[inline(always)]
    pub fn tsevntena(&self) -> TSEVNTENA_R {
        TSEVNTENA_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - TSMSTRENA"]
    #[inline(always)]
    pub fn tsmstrena(&self) -> TSMSTRENA_R {
        TSMSTRENA_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:17 - SNAPTYPSEL"]
    #[inline(always)]
    pub fn snaptypsel(&self) -> SNAPTYPSEL_R {
        SNAPTYPSEL_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 18 - TSENMACADDR"]
    #[inline(always)]
    pub fn tsenmacaddr(&self) -> TSENMACADDR_R {
        TSENMACADDR_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - CSC"]
    #[inline(always)]
    pub fn csc(&self) -> CSC_R {
        CSC_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 24 - TXTSSTSM"]
    #[inline(always)]
    pub fn txtsstsm(&self) -> TXTSSTSM_R {
        TXTSSTSM_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 28 - AV8021ASMEN"]
    #[inline(always)]
    pub fn av8021asmen(&self) -> AV8021ASMEN_R {
        AV8021ASMEN_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TSENA"]
    #[inline(always)]
    #[must_use]
    pub fn tsena(&mut self) -> TSENA_W<0> {
        TSENA_W::new(self)
    }
    #[doc = "Bit 1 - TSCFUPDT"]
    #[inline(always)]
    #[must_use]
    pub fn tscfupdt(&mut self) -> TSCFUPDT_W<1> {
        TSCFUPDT_W::new(self)
    }
    #[doc = "Bit 2 - TSINIT"]
    #[inline(always)]
    #[must_use]
    pub fn tsinit(&mut self) -> TSINIT_W<2> {
        TSINIT_W::new(self)
    }
    #[doc = "Bit 3 - TSUPDT"]
    #[inline(always)]
    #[must_use]
    pub fn tsupdt(&mut self) -> TSUPDT_W<3> {
        TSUPDT_W::new(self)
    }
    #[doc = "Bit 5 - TSADDREG"]
    #[inline(always)]
    #[must_use]
    pub fn tsaddreg(&mut self) -> TSADDREG_W<5> {
        TSADDREG_W::new(self)
    }
    #[doc = "Bit 8 - TSENALL"]
    #[inline(always)]
    #[must_use]
    pub fn tsenall(&mut self) -> TSENALL_W<8> {
        TSENALL_W::new(self)
    }
    #[doc = "Bit 9 - TSCTRLSSR"]
    #[inline(always)]
    #[must_use]
    pub fn tsctrlssr(&mut self) -> TSCTRLSSR_W<9> {
        TSCTRLSSR_W::new(self)
    }
    #[doc = "Bit 10 - TSVER2ENA"]
    #[inline(always)]
    #[must_use]
    pub fn tsver2ena(&mut self) -> TSVER2ENA_W<10> {
        TSVER2ENA_W::new(self)
    }
    #[doc = "Bit 11 - TSIPENA"]
    #[inline(always)]
    #[must_use]
    pub fn tsipena(&mut self) -> TSIPENA_W<11> {
        TSIPENA_W::new(self)
    }
    #[doc = "Bit 12 - TSIPV6ENA"]
    #[inline(always)]
    #[must_use]
    pub fn tsipv6ena(&mut self) -> TSIPV6ENA_W<12> {
        TSIPV6ENA_W::new(self)
    }
    #[doc = "Bit 13 - TSIPV4ENA"]
    #[inline(always)]
    #[must_use]
    pub fn tsipv4ena(&mut self) -> TSIPV4ENA_W<13> {
        TSIPV4ENA_W::new(self)
    }
    #[doc = "Bit 14 - TSEVNTENA"]
    #[inline(always)]
    #[must_use]
    pub fn tsevntena(&mut self) -> TSEVNTENA_W<14> {
        TSEVNTENA_W::new(self)
    }
    #[doc = "Bit 15 - TSMSTRENA"]
    #[inline(always)]
    #[must_use]
    pub fn tsmstrena(&mut self) -> TSMSTRENA_W<15> {
        TSMSTRENA_W::new(self)
    }
    #[doc = "Bits 16:17 - SNAPTYPSEL"]
    #[inline(always)]
    #[must_use]
    pub fn snaptypsel(&mut self) -> SNAPTYPSEL_W<16> {
        SNAPTYPSEL_W::new(self)
    }
    #[doc = "Bit 18 - TSENMACADDR"]
    #[inline(always)]
    #[must_use]
    pub fn tsenmacaddr(&mut self) -> TSENMACADDR_W<18> {
        TSENMACADDR_W::new(self)
    }
    #[doc = "Bit 24 - TXTSSTSM"]
    #[inline(always)]
    #[must_use]
    pub fn txtsstsm(&mut self) -> TXTSSTSM_W<24> {
        TXTSSTSM_W::new(self)
    }
    #[doc = "Bit 28 - AV8021ASMEN"]
    #[inline(always)]
    #[must_use]
    pub fn av8021asmen(&mut self) -> AV8021ASMEN_W<28> {
        AV8021ASMEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register controls the operation of the System Time generator and processing of PTP packets for timestamping in the Receiver.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_mactscr](index.html) module"]
pub struct ETH_MACTSCR_SPEC;
impl crate::RegisterSpec for ETH_MACTSCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eth_mactscr::R](R) reader structure"]
impl crate::Readable for ETH_MACTSCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eth_mactscr::W](W) writer structure"]
impl crate::Writable for ETH_MACTSCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ETH_MACTSCR to value 0x2000"]
impl crate::Resettable for ETH_MACTSCR_SPEC {
    const RESET_VALUE: Self::Ux = 0x2000;
}
