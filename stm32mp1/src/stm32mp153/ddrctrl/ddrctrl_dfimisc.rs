#[doc = "Register `DDRCTRL_DFIMISC` reader"]
pub struct R(crate::R<DDRCTRL_DFIMISC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DDRCTRL_DFIMISC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DDRCTRL_DFIMISC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DDRCTRL_DFIMISC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DDRCTRL_DFIMISC` writer"]
pub struct W(crate::W<DDRCTRL_DFIMISC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DDRCTRL_DFIMISC_SPEC>;
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
impl From<crate::W<DDRCTRL_DFIMISC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DDRCTRL_DFIMISC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DFI_INIT_COMPLETE_EN` reader - DFI_INIT_COMPLETE_EN"]
pub type DFI_INIT_COMPLETE_EN_R = crate::BitReader<bool>;
#[doc = "Field `DFI_INIT_COMPLETE_EN` writer - DFI_INIT_COMPLETE_EN"]
pub type DFI_INIT_COMPLETE_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DDRCTRL_DFIMISC_SPEC, bool, O>;
#[doc = "Field `CTL_IDLE_EN` reader - CTL_IDLE_EN"]
pub type CTL_IDLE_EN_R = crate::BitReader<bool>;
#[doc = "Field `CTL_IDLE_EN` writer - CTL_IDLE_EN"]
pub type CTL_IDLE_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DDRCTRL_DFIMISC_SPEC, bool, O>;
#[doc = "Field `DFI_INIT_START` reader - DFI_INIT_START"]
pub type DFI_INIT_START_R = crate::BitReader<bool>;
#[doc = "Field `DFI_INIT_START` writer - DFI_INIT_START"]
pub type DFI_INIT_START_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DDRCTRL_DFIMISC_SPEC, bool, O>;
#[doc = "Field `DFI_FREQUENCY` reader - DFI_FREQUENCY"]
pub type DFI_FREQUENCY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DFI_FREQUENCY` writer - DFI_FREQUENCY"]
pub type DFI_FREQUENCY_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DDRCTRL_DFIMISC_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bit 0 - DFI_INIT_COMPLETE_EN"]
    #[inline(always)]
    pub fn dfi_init_complete_en(&self) -> DFI_INIT_COMPLETE_EN_R {
        DFI_INIT_COMPLETE_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - CTL_IDLE_EN"]
    #[inline(always)]
    pub fn ctl_idle_en(&self) -> CTL_IDLE_EN_R {
        CTL_IDLE_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - DFI_INIT_START"]
    #[inline(always)]
    pub fn dfi_init_start(&self) -> DFI_INIT_START_R {
        DFI_INIT_START_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 8:12 - DFI_FREQUENCY"]
    #[inline(always)]
    pub fn dfi_frequency(&self) -> DFI_FREQUENCY_R {
        DFI_FREQUENCY_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - DFI_INIT_COMPLETE_EN"]
    #[inline(always)]
    #[must_use]
    pub fn dfi_init_complete_en(&mut self) -> DFI_INIT_COMPLETE_EN_W<0> {
        DFI_INIT_COMPLETE_EN_W::new(self)
    }
    #[doc = "Bit 4 - CTL_IDLE_EN"]
    #[inline(always)]
    #[must_use]
    pub fn ctl_idle_en(&mut self) -> CTL_IDLE_EN_W<4> {
        CTL_IDLE_EN_W::new(self)
    }
    #[doc = "Bit 5 - DFI_INIT_START"]
    #[inline(always)]
    #[must_use]
    pub fn dfi_init_start(&mut self) -> DFI_INIT_START_W<5> {
        DFI_INIT_START_W::new(self)
    }
    #[doc = "Bits 8:12 - DFI_FREQUENCY"]
    #[inline(always)]
    #[must_use]
    pub fn dfi_frequency(&mut self) -> DFI_FREQUENCY_W<8> {
        DFI_FREQUENCY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DDRCTRL DFI miscellaneous control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrctrl_dfimisc](index.html) module"]
pub struct DDRCTRL_DFIMISC_SPEC;
impl crate::RegisterSpec for DDRCTRL_DFIMISC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ddrctrl_dfimisc::R](R) reader structure"]
impl crate::Readable for DDRCTRL_DFIMISC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ddrctrl_dfimisc::W](W) writer structure"]
impl crate::Writable for DDRCTRL_DFIMISC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DDRCTRL_DFIMISC to value 0x01"]
impl crate::Resettable for DDRCTRL_DFIMISC_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
