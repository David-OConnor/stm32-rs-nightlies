#[doc = "Register `DDRPHYC_PIR` writer"]
pub struct W(crate::W<DDRPHYC_PIR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DDRPHYC_PIR_SPEC>;
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
impl From<crate::W<DDRPHYC_PIR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DDRPHYC_PIR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INIT` writer - INIT"]
pub type INIT_W<'a, const O: u8> = crate::BitWriter<'a, u32, DDRPHYC_PIR_SPEC, bool, O>;
#[doc = "Field `DLLSRST` writer - DLLSRST"]
pub type DLLSRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, DDRPHYC_PIR_SPEC, bool, O>;
#[doc = "Field `DLLLOCK` writer - DLLLOCK"]
pub type DLLLOCK_W<'a, const O: u8> = crate::BitWriter<'a, u32, DDRPHYC_PIR_SPEC, bool, O>;
#[doc = "Field `ZCAL` writer - ZCAL"]
pub type ZCAL_W<'a, const O: u8> = crate::BitWriter<'a, u32, DDRPHYC_PIR_SPEC, bool, O>;
#[doc = "Field `ITMSRST` writer - ITMSRST"]
pub type ITMSRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, DDRPHYC_PIR_SPEC, bool, O>;
#[doc = "Field `DRAMRST` writer - DRAMRST"]
pub type DRAMRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, DDRPHYC_PIR_SPEC, bool, O>;
#[doc = "Field `DRAMINIT` writer - DRAMINIT"]
pub type DRAMINIT_W<'a, const O: u8> = crate::BitWriter<'a, u32, DDRPHYC_PIR_SPEC, bool, O>;
#[doc = "Field `QSTRN` writer - QSTRN"]
pub type QSTRN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DDRPHYC_PIR_SPEC, bool, O>;
#[doc = "Field `RVTRN` writer - RVTRN"]
pub type RVTRN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DDRPHYC_PIR_SPEC, bool, O>;
#[doc = "Field `ICPC` writer - ICPC"]
pub type ICPC_W<'a, const O: u8> = crate::BitWriter<'a, u32, DDRPHYC_PIR_SPEC, bool, O>;
#[doc = "Field `DLLBYP` writer - DLLBYP"]
pub type DLLBYP_W<'a, const O: u8> = crate::BitWriter<'a, u32, DDRPHYC_PIR_SPEC, bool, O>;
#[doc = "Field `CTLDINIT` writer - CTLDINIT"]
pub type CTLDINIT_W<'a, const O: u8> = crate::BitWriter<'a, u32, DDRPHYC_PIR_SPEC, bool, O>;
#[doc = "Field `CLRSR` writer - CLRSR"]
pub type CLRSR_W<'a, const O: u8> = crate::BitWriter<'a, u32, DDRPHYC_PIR_SPEC, bool, O>;
#[doc = "Field `LOCKBYP` writer - LOCKBYP"]
pub type LOCKBYP_W<'a, const O: u8> = crate::BitWriter<'a, u32, DDRPHYC_PIR_SPEC, bool, O>;
#[doc = "Field `ZCALBYP` writer - ZCALBYP"]
pub type ZCALBYP_W<'a, const O: u8> = crate::BitWriter<'a, u32, DDRPHYC_PIR_SPEC, bool, O>;
#[doc = "Field `INITBYP` writer - INITBYP"]
pub type INITBYP_W<'a, const O: u8> = crate::BitWriter<'a, u32, DDRPHYC_PIR_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - INIT"]
    #[inline(always)]
    #[must_use]
    pub fn init(&mut self) -> INIT_W<0> {
        INIT_W::new(self)
    }
    #[doc = "Bit 1 - DLLSRST"]
    #[inline(always)]
    #[must_use]
    pub fn dllsrst(&mut self) -> DLLSRST_W<1> {
        DLLSRST_W::new(self)
    }
    #[doc = "Bit 2 - DLLLOCK"]
    #[inline(always)]
    #[must_use]
    pub fn dlllock(&mut self) -> DLLLOCK_W<2> {
        DLLLOCK_W::new(self)
    }
    #[doc = "Bit 3 - ZCAL"]
    #[inline(always)]
    #[must_use]
    pub fn zcal(&mut self) -> ZCAL_W<3> {
        ZCAL_W::new(self)
    }
    #[doc = "Bit 4 - ITMSRST"]
    #[inline(always)]
    #[must_use]
    pub fn itmsrst(&mut self) -> ITMSRST_W<4> {
        ITMSRST_W::new(self)
    }
    #[doc = "Bit 5 - DRAMRST"]
    #[inline(always)]
    #[must_use]
    pub fn dramrst(&mut self) -> DRAMRST_W<5> {
        DRAMRST_W::new(self)
    }
    #[doc = "Bit 6 - DRAMINIT"]
    #[inline(always)]
    #[must_use]
    pub fn draminit(&mut self) -> DRAMINIT_W<6> {
        DRAMINIT_W::new(self)
    }
    #[doc = "Bit 7 - QSTRN"]
    #[inline(always)]
    #[must_use]
    pub fn qstrn(&mut self) -> QSTRN_W<7> {
        QSTRN_W::new(self)
    }
    #[doc = "Bit 8 - RVTRN"]
    #[inline(always)]
    #[must_use]
    pub fn rvtrn(&mut self) -> RVTRN_W<8> {
        RVTRN_W::new(self)
    }
    #[doc = "Bit 16 - ICPC"]
    #[inline(always)]
    #[must_use]
    pub fn icpc(&mut self) -> ICPC_W<16> {
        ICPC_W::new(self)
    }
    #[doc = "Bit 17 - DLLBYP"]
    #[inline(always)]
    #[must_use]
    pub fn dllbyp(&mut self) -> DLLBYP_W<17> {
        DLLBYP_W::new(self)
    }
    #[doc = "Bit 18 - CTLDINIT"]
    #[inline(always)]
    #[must_use]
    pub fn ctldinit(&mut self) -> CTLDINIT_W<18> {
        CTLDINIT_W::new(self)
    }
    #[doc = "Bit 28 - CLRSR"]
    #[inline(always)]
    #[must_use]
    pub fn clrsr(&mut self) -> CLRSR_W<28> {
        CLRSR_W::new(self)
    }
    #[doc = "Bit 29 - LOCKBYP"]
    #[inline(always)]
    #[must_use]
    pub fn lockbyp(&mut self) -> LOCKBYP_W<29> {
        LOCKBYP_W::new(self)
    }
    #[doc = "Bit 30 - ZCALBYP"]
    #[inline(always)]
    #[must_use]
    pub fn zcalbyp(&mut self) -> ZCALBYP_W<30> {
        ZCALBYP_W::new(self)
    }
    #[doc = "Bit 31 - INITBYP"]
    #[inline(always)]
    #[must_use]
    pub fn initbyp(&mut self) -> INITBYP_W<31> {
        INITBYP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DDRPHYC PHY initialization register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrphyc_pir](index.html) module"]
pub struct DDRPHYC_PIR_SPEC;
impl crate::RegisterSpec for DDRPHYC_PIR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [ddrphyc_pir::W](W) writer structure"]
impl crate::Writable for DDRPHYC_PIR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DDRPHYC_PIR to value 0"]
impl crate::Resettable for DDRPHYC_PIR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
