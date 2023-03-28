#[doc = "Register `DDRCTRL_CRCPARCTL0` reader"]
pub struct R(crate::R<DDRCTRL_CRCPARCTL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DDRCTRL_CRCPARCTL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DDRCTRL_CRCPARCTL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DDRCTRL_CRCPARCTL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DDRCTRL_CRCPARCTL0` writer"]
pub struct W(crate::W<DDRCTRL_CRCPARCTL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DDRCTRL_CRCPARCTL0_SPEC>;
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
impl From<crate::W<DDRCTRL_CRCPARCTL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DDRCTRL_CRCPARCTL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DFI_ALERT_ERR_INT_EN` reader - DFI_ALERT_ERR_INT_EN"]
pub type DFI_ALERT_ERR_INT_EN_R = crate::BitReader<bool>;
#[doc = "Field `DFI_ALERT_ERR_INT_EN` writer - DFI_ALERT_ERR_INT_EN"]
pub type DFI_ALERT_ERR_INT_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DDRCTRL_CRCPARCTL0_SPEC, bool, O>;
#[doc = "Field `DFI_ALERT_ERR_INT_CLR` reader - DFI_ALERT_ERR_INT_CLR"]
pub type DFI_ALERT_ERR_INT_CLR_R = crate::BitReader<bool>;
#[doc = "Field `DFI_ALERT_ERR_INT_CLR` writer - DFI_ALERT_ERR_INT_CLR"]
pub type DFI_ALERT_ERR_INT_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DDRCTRL_CRCPARCTL0_SPEC, bool, O>;
#[doc = "Field `DFI_ALERT_ERR_CNT_CLR` reader - DFI_ALERT_ERR_CNT_CLR"]
pub type DFI_ALERT_ERR_CNT_CLR_R = crate::BitReader<bool>;
#[doc = "Field `DFI_ALERT_ERR_CNT_CLR` writer - DFI_ALERT_ERR_CNT_CLR"]
pub type DFI_ALERT_ERR_CNT_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DDRCTRL_CRCPARCTL0_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - DFI_ALERT_ERR_INT_EN"]
    #[inline(always)]
    pub fn dfi_alert_err_int_en(&self) -> DFI_ALERT_ERR_INT_EN_R {
        DFI_ALERT_ERR_INT_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DFI_ALERT_ERR_INT_CLR"]
    #[inline(always)]
    pub fn dfi_alert_err_int_clr(&self) -> DFI_ALERT_ERR_INT_CLR_R {
        DFI_ALERT_ERR_INT_CLR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DFI_ALERT_ERR_CNT_CLR"]
    #[inline(always)]
    pub fn dfi_alert_err_cnt_clr(&self) -> DFI_ALERT_ERR_CNT_CLR_R {
        DFI_ALERT_ERR_CNT_CLR_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DFI_ALERT_ERR_INT_EN"]
    #[inline(always)]
    #[must_use]
    pub fn dfi_alert_err_int_en(&mut self) -> DFI_ALERT_ERR_INT_EN_W<0> {
        DFI_ALERT_ERR_INT_EN_W::new(self)
    }
    #[doc = "Bit 1 - DFI_ALERT_ERR_INT_CLR"]
    #[inline(always)]
    #[must_use]
    pub fn dfi_alert_err_int_clr(&mut self) -> DFI_ALERT_ERR_INT_CLR_W<1> {
        DFI_ALERT_ERR_INT_CLR_W::new(self)
    }
    #[doc = "Bit 2 - DFI_ALERT_ERR_CNT_CLR"]
    #[inline(always)]
    #[must_use]
    pub fn dfi_alert_err_cnt_clr(&mut self) -> DFI_ALERT_ERR_CNT_CLR_W<2> {
        DFI_ALERT_ERR_CNT_CLR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DDRCTRL CRC parity control register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrctrl_crcparctl0](index.html) module"]
pub struct DDRCTRL_CRCPARCTL0_SPEC;
impl crate::RegisterSpec for DDRCTRL_CRCPARCTL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ddrctrl_crcparctl0::R](R) reader structure"]
impl crate::Readable for DDRCTRL_CRCPARCTL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ddrctrl_crcparctl0::W](W) writer structure"]
impl crate::Writable for DDRCTRL_CRCPARCTL0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DDRCTRL_CRCPARCTL0 to value 0"]
impl crate::Resettable for DDRCTRL_CRCPARCTL0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
