#[doc = "Register `DDRCTRL_DFIUPD1` reader"]
pub struct R(crate::R<DDRCTRL_DFIUPD1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DDRCTRL_DFIUPD1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DDRCTRL_DFIUPD1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DDRCTRL_DFIUPD1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DDRCTRL_DFIUPD1` writer"]
pub struct W(crate::W<DDRCTRL_DFIUPD1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DDRCTRL_DFIUPD1_SPEC>;
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
impl From<crate::W<DDRCTRL_DFIUPD1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DDRCTRL_DFIUPD1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DFI_T_CTRLUPD_INTERVAL_MAX_X1024` reader - DFI_T_CTRLUPD_INTERVAL_MAX_X1024"]
pub type DFI_T_CTRLUPD_INTERVAL_MAX_X1024_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DFI_T_CTRLUPD_INTERVAL_MAX_X1024` writer - DFI_T_CTRLUPD_INTERVAL_MAX_X1024"]
pub type DFI_T_CTRLUPD_INTERVAL_MAX_X1024_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DDRCTRL_DFIUPD1_SPEC, u8, u8, 8, O>;
#[doc = "Field `DFI_T_CTRLUPD_INTERVAL_MIN_X1024` reader - DFI_T_CTRLUPD_INTERVAL_MIN_X1024"]
pub type DFI_T_CTRLUPD_INTERVAL_MIN_X1024_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DFI_T_CTRLUPD_INTERVAL_MIN_X1024` writer - DFI_T_CTRLUPD_INTERVAL_MIN_X1024"]
pub type DFI_T_CTRLUPD_INTERVAL_MIN_X1024_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DDRCTRL_DFIUPD1_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - DFI_T_CTRLUPD_INTERVAL_MAX_X1024"]
    #[inline(always)]
    pub fn dfi_t_ctrlupd_interval_max_x1024(&self) -> DFI_T_CTRLUPD_INTERVAL_MAX_X1024_R {
        DFI_T_CTRLUPD_INTERVAL_MAX_X1024_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - DFI_T_CTRLUPD_INTERVAL_MIN_X1024"]
    #[inline(always)]
    pub fn dfi_t_ctrlupd_interval_min_x1024(&self) -> DFI_T_CTRLUPD_INTERVAL_MIN_X1024_R {
        DFI_T_CTRLUPD_INTERVAL_MIN_X1024_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - DFI_T_CTRLUPD_INTERVAL_MAX_X1024"]
    #[inline(always)]
    #[must_use]
    pub fn dfi_t_ctrlupd_interval_max_x1024(&mut self) -> DFI_T_CTRLUPD_INTERVAL_MAX_X1024_W<0> {
        DFI_T_CTRLUPD_INTERVAL_MAX_X1024_W::new(self)
    }
    #[doc = "Bits 16:23 - DFI_T_CTRLUPD_INTERVAL_MIN_X1024"]
    #[inline(always)]
    #[must_use]
    pub fn dfi_t_ctrlupd_interval_min_x1024(&mut self) -> DFI_T_CTRLUPD_INTERVAL_MIN_X1024_W<16> {
        DFI_T_CTRLUPD_INTERVAL_MIN_X1024_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DDRCTRL DFI update register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrctrl_dfiupd1](index.html) module"]
pub struct DDRCTRL_DFIUPD1_SPEC;
impl crate::RegisterSpec for DDRCTRL_DFIUPD1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ddrctrl_dfiupd1::R](R) reader structure"]
impl crate::Readable for DDRCTRL_DFIUPD1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ddrctrl_dfiupd1::W](W) writer structure"]
impl crate::Writable for DDRCTRL_DFIUPD1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DDRCTRL_DFIUPD1 to value 0x0001_0001"]
impl crate::Resettable for DDRCTRL_DFIUPD1_SPEC {
    const RESET_VALUE: Self::Ux = 0x0001_0001;
}
