#[doc = "Register `I3C_CEVR` writer"]
pub struct W(crate::W<I3C_CEVR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<I3C_CEVR_SPEC>;
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
impl From<crate::W<I3C_CEVR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<I3C_CEVR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CFCF` writer - clear frame complete flag (whatever the I3C is acting as controller/target)"]
pub type CFCF_W<'a, const O: u8> = crate::BitWriter<'a, u32, I3C_CEVR_SPEC, bool, O>;
#[doc = "Field `CRXTGTENDF` writer - clear target-initiated read end flag (when the I3C is acting as controller)"]
pub type CRXTGTENDF_W<'a, const O: u8> = crate::BitWriter<'a, u32, I3C_CEVR_SPEC, bool, O>;
#[doc = "Field `CERRF` writer - clear error flag (whatever the I3C is acting as controller/target)"]
pub type CERRF_W<'a, const O: u8> = crate::BitWriter<'a, u32, I3C_CEVR_SPEC, bool, O>;
#[doc = "Field `CIBIF` writer - clear IBI request flag (when the I3C is acting as controller)"]
pub type CIBIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, I3C_CEVR_SPEC, bool, O>;
#[doc = "Field `CIBIENDF` writer - clear IBI end flag (when the I3C is acting as target)"]
pub type CIBIENDF_W<'a, const O: u8> = crate::BitWriter<'a, u32, I3C_CEVR_SPEC, bool, O>;
#[doc = "Field `CCRF` writer - clear controller-role request flag (when the I3C is acting as controller)"]
pub type CCRF_W<'a, const O: u8> = crate::BitWriter<'a, u32, I3C_CEVR_SPEC, bool, O>;
#[doc = "Field `CCRUPDF` writer - clear controller-role update flag (when the I3C is acting as target)"]
pub type CCRUPDF_W<'a, const O: u8> = crate::BitWriter<'a, u32, I3C_CEVR_SPEC, bool, O>;
#[doc = "Field `CHJF` writer - clear hot-join flag (when the I3C is acting as controller)"]
pub type CHJF_W<'a, const O: u8> = crate::BitWriter<'a, u32, I3C_CEVR_SPEC, bool, O>;
#[doc = "Field `CWKPF` writer - clear wakeup flag (when the I3C is acting as target)"]
pub type CWKPF_W<'a, const O: u8> = crate::BitWriter<'a, u32, I3C_CEVR_SPEC, bool, O>;
#[doc = "Field `CGETF` writer - clear GETxxx CCC flag (when the I3C is acting as target)"]
pub type CGETF_W<'a, const O: u8> = crate::BitWriter<'a, u32, I3C_CEVR_SPEC, bool, O>;
#[doc = "Field `CSTAF` writer - clear GETSTATUS CCC flag (when the I3C is acting as target)"]
pub type CSTAF_W<'a, const O: u8> = crate::BitWriter<'a, u32, I3C_CEVR_SPEC, bool, O>;
#[doc = "Field `CDAUPDF` writer - clear ENTDAA/RSTDAA/SETNEWDA CCC flag (when the I3C is acting as target)"]
pub type CDAUPDF_W<'a, const O: u8> = crate::BitWriter<'a, u32, I3C_CEVR_SPEC, bool, O>;
#[doc = "Field `CMWLUPDF` writer - clear SETMWL CCC flag (when the I3C is acting as target)"]
pub type CMWLUPDF_W<'a, const O: u8> = crate::BitWriter<'a, u32, I3C_CEVR_SPEC, bool, O>;
#[doc = "Field `CMRLUPDF` writer - clear SETMRL CCC flag (when the I3C is acting as target)"]
pub type CMRLUPDF_W<'a, const O: u8> = crate::BitWriter<'a, u32, I3C_CEVR_SPEC, bool, O>;
#[doc = "Field `CRSTF` writer - clear reset pattern flag (when the I3C is acting as target)"]
pub type CRSTF_W<'a, const O: u8> = crate::BitWriter<'a, u32, I3C_CEVR_SPEC, bool, O>;
#[doc = "Field `CASUPDF` writer - clear ENTASx CCC flag (when the I3C is acting as target)"]
pub type CASUPDF_W<'a, const O: u8> = crate::BitWriter<'a, u32, I3C_CEVR_SPEC, bool, O>;
#[doc = "Field `CINTUPDF` writer - clear ENEC/DISEC CCC flag (when the I3C is acting as target)"]
pub type CINTUPDF_W<'a, const O: u8> = crate::BitWriter<'a, u32, I3C_CEVR_SPEC, bool, O>;
#[doc = "Field `CDEFF` writer - clear DEFTGTS CCC flag (when the I3C is acting as target)"]
pub type CDEFF_W<'a, const O: u8> = crate::BitWriter<'a, u32, I3C_CEVR_SPEC, bool, O>;
#[doc = "Field `CGRPF` writer - clear DEFGRPA CCC flag (when the I3C is acting as target)"]
pub type CGRPF_W<'a, const O: u8> = crate::BitWriter<'a, u32, I3C_CEVR_SPEC, bool, O>;
impl W {
    #[doc = "Bit 9 - clear frame complete flag (whatever the I3C is acting as controller/target)"]
    #[inline(always)]
    #[must_use]
    pub fn cfcf(&mut self) -> CFCF_W<9> {
        CFCF_W::new(self)
    }
    #[doc = "Bit 10 - clear target-initiated read end flag (when the I3C is acting as controller)"]
    #[inline(always)]
    #[must_use]
    pub fn crxtgtendf(&mut self) -> CRXTGTENDF_W<10> {
        CRXTGTENDF_W::new(self)
    }
    #[doc = "Bit 11 - clear error flag (whatever the I3C is acting as controller/target)"]
    #[inline(always)]
    #[must_use]
    pub fn cerrf(&mut self) -> CERRF_W<11> {
        CERRF_W::new(self)
    }
    #[doc = "Bit 15 - clear IBI request flag (when the I3C is acting as controller)"]
    #[inline(always)]
    #[must_use]
    pub fn cibif(&mut self) -> CIBIF_W<15> {
        CIBIF_W::new(self)
    }
    #[doc = "Bit 16 - clear IBI end flag (when the I3C is acting as target)"]
    #[inline(always)]
    #[must_use]
    pub fn cibiendf(&mut self) -> CIBIENDF_W<16> {
        CIBIENDF_W::new(self)
    }
    #[doc = "Bit 17 - clear controller-role request flag (when the I3C is acting as controller)"]
    #[inline(always)]
    #[must_use]
    pub fn ccrf(&mut self) -> CCRF_W<17> {
        CCRF_W::new(self)
    }
    #[doc = "Bit 18 - clear controller-role update flag (when the I3C is acting as target)"]
    #[inline(always)]
    #[must_use]
    pub fn ccrupdf(&mut self) -> CCRUPDF_W<18> {
        CCRUPDF_W::new(self)
    }
    #[doc = "Bit 19 - clear hot-join flag (when the I3C is acting as controller)"]
    #[inline(always)]
    #[must_use]
    pub fn chjf(&mut self) -> CHJF_W<19> {
        CHJF_W::new(self)
    }
    #[doc = "Bit 21 - clear wakeup flag (when the I3C is acting as target)"]
    #[inline(always)]
    #[must_use]
    pub fn cwkpf(&mut self) -> CWKPF_W<21> {
        CWKPF_W::new(self)
    }
    #[doc = "Bit 22 - clear GETxxx CCC flag (when the I3C is acting as target)"]
    #[inline(always)]
    #[must_use]
    pub fn cgetf(&mut self) -> CGETF_W<22> {
        CGETF_W::new(self)
    }
    #[doc = "Bit 23 - clear GETSTATUS CCC flag (when the I3C is acting as target)"]
    #[inline(always)]
    #[must_use]
    pub fn cstaf(&mut self) -> CSTAF_W<23> {
        CSTAF_W::new(self)
    }
    #[doc = "Bit 24 - clear ENTDAA/RSTDAA/SETNEWDA CCC flag (when the I3C is acting as target)"]
    #[inline(always)]
    #[must_use]
    pub fn cdaupdf(&mut self) -> CDAUPDF_W<24> {
        CDAUPDF_W::new(self)
    }
    #[doc = "Bit 25 - clear SETMWL CCC flag (when the I3C is acting as target)"]
    #[inline(always)]
    #[must_use]
    pub fn cmwlupdf(&mut self) -> CMWLUPDF_W<25> {
        CMWLUPDF_W::new(self)
    }
    #[doc = "Bit 26 - clear SETMRL CCC flag (when the I3C is acting as target)"]
    #[inline(always)]
    #[must_use]
    pub fn cmrlupdf(&mut self) -> CMRLUPDF_W<26> {
        CMRLUPDF_W::new(self)
    }
    #[doc = "Bit 27 - clear reset pattern flag (when the I3C is acting as target)"]
    #[inline(always)]
    #[must_use]
    pub fn crstf(&mut self) -> CRSTF_W<27> {
        CRSTF_W::new(self)
    }
    #[doc = "Bit 28 - clear ENTASx CCC flag (when the I3C is acting as target)"]
    #[inline(always)]
    #[must_use]
    pub fn casupdf(&mut self) -> CASUPDF_W<28> {
        CASUPDF_W::new(self)
    }
    #[doc = "Bit 29 - clear ENEC/DISEC CCC flag (when the I3C is acting as target)"]
    #[inline(always)]
    #[must_use]
    pub fn cintupdf(&mut self) -> CINTUPDF_W<29> {
        CINTUPDF_W::new(self)
    }
    #[doc = "Bit 30 - clear DEFTGTS CCC flag (when the I3C is acting as target)"]
    #[inline(always)]
    #[must_use]
    pub fn cdeff(&mut self) -> CDEFF_W<30> {
        CDEFF_W::new(self)
    }
    #[doc = "Bit 31 - clear DEFGRPA CCC flag (when the I3C is acting as target)"]
    #[inline(always)]
    #[must_use]
    pub fn cgrpf(&mut self) -> CGRPF_W<31> {
        CGRPF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I3C clear event register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i3c_cevr](index.html) module"]
pub struct I3C_CEVR_SPEC;
impl crate::RegisterSpec for I3C_CEVR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [i3c_cevr::W](W) writer structure"]
impl crate::Writable for I3C_CEVR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets I3C_CEVR to value 0"]
impl crate::Resettable for I3C_CEVR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
