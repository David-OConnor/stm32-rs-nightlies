#[doc = "Register `BDCR` reader"]
pub struct R(crate::R<BDCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BDCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BDCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BDCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BDCR` writer"]
pub struct W(crate::W<BDCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BDCR_SPEC>;
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
impl From<crate::W<BDCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BDCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BREN` reader - Backup RAM retention in Standby and V<sub>BAT</sub> modes When this bit set, the backup regulator (used to maintain the backup RAM content in Standby and V<sub>BAT</sub> modes) is enabled. If BREN is cleared, the backup regulator is switched off. The backup RAM can still be used in Run and Stop modes. However its content is lost in Standby and V<sub>BAT</sub> modes. If BREN is set, the application must wait till the backup regulator ready flag (BRRDY) is set to indicate that the data written into the SRAM is maintained in Standby and V<sub>BAT</sub> modes."]
pub type BREN_R = crate::BitReader<bool>;
#[doc = "Field `BREN` writer - Backup RAM retention in Standby and V<sub>BAT</sub> modes When this bit set, the backup regulator (used to maintain the backup RAM content in Standby and V<sub>BAT</sub> modes) is enabled. If BREN is cleared, the backup regulator is switched off. The backup RAM can still be used in Run and Stop modes. However its content is lost in Standby and V<sub>BAT</sub> modes. If BREN is set, the application must wait till the backup regulator ready flag (BRRDY) is set to indicate that the data written into the SRAM is maintained in Standby and V<sub>BAT</sub> modes."]
pub type BREN_W<'a, const O: u8> = crate::BitWriter<'a, u32, BDCR_SPEC, bool, O>;
#[doc = "Field `MONEN` reader - Backup domain voltage and temperature monitoring enable"]
pub type MONEN_R = crate::BitReader<bool>;
#[doc = "Field `MONEN` writer - Backup domain voltage and temperature monitoring enable"]
pub type MONEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, BDCR_SPEC, bool, O>;
#[doc = "Field `VBE` reader - V<sub>BAT</sub> charging enable Note: Reset only by POR,."]
pub type VBE_R = crate::BitReader<bool>;
#[doc = "Field `VBE` writer - V<sub>BAT</sub> charging enable Note: Reset only by POR,."]
pub type VBE_W<'a, const O: u8> = crate::BitWriter<'a, u32, BDCR_SPEC, bool, O>;
#[doc = "Field `VBRS` reader - V<sub>BAT</sub> charging resistor selection"]
pub type VBRS_R = crate::BitReader<bool>;
#[doc = "Field `VBRS` writer - V<sub>BAT</sub> charging resistor selection"]
pub type VBRS_W<'a, const O: u8> = crate::BitWriter<'a, u32, BDCR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Backup RAM retention in Standby and V<sub>BAT</sub> modes When this bit set, the backup regulator (used to maintain the backup RAM content in Standby and V<sub>BAT</sub> modes) is enabled. If BREN is cleared, the backup regulator is switched off. The backup RAM can still be used in Run and Stop modes. However its content is lost in Standby and V<sub>BAT</sub> modes. If BREN is set, the application must wait till the backup regulator ready flag (BRRDY) is set to indicate that the data written into the SRAM is maintained in Standby and V<sub>BAT</sub> modes."]
    #[inline(always)]
    pub fn bren(&self) -> BREN_R {
        BREN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Backup domain voltage and temperature monitoring enable"]
    #[inline(always)]
    pub fn monen(&self) -> MONEN_R {
        MONEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - V<sub>BAT</sub> charging enable Note: Reset only by POR,."]
    #[inline(always)]
    pub fn vbe(&self) -> VBE_R {
        VBE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - V<sub>BAT</sub> charging resistor selection"]
    #[inline(always)]
    pub fn vbrs(&self) -> VBRS_R {
        VBRS_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Backup RAM retention in Standby and V<sub>BAT</sub> modes When this bit set, the backup regulator (used to maintain the backup RAM content in Standby and V<sub>BAT</sub> modes) is enabled. If BREN is cleared, the backup regulator is switched off. The backup RAM can still be used in Run and Stop modes. However its content is lost in Standby and V<sub>BAT</sub> modes. If BREN is set, the application must wait till the backup regulator ready flag (BRRDY) is set to indicate that the data written into the SRAM is maintained in Standby and V<sub>BAT</sub> modes."]
    #[inline(always)]
    #[must_use]
    pub fn bren(&mut self) -> BREN_W<0> {
        BREN_W::new(self)
    }
    #[doc = "Bit 1 - Backup domain voltage and temperature monitoring enable"]
    #[inline(always)]
    #[must_use]
    pub fn monen(&mut self) -> MONEN_W<1> {
        MONEN_W::new(self)
    }
    #[doc = "Bit 8 - V<sub>BAT</sub> charging enable Note: Reset only by POR,."]
    #[inline(always)]
    #[must_use]
    pub fn vbe(&mut self) -> VBE_W<8> {
        VBE_W::new(self)
    }
    #[doc = "Bit 9 - V<sub>BAT</sub> charging resistor selection"]
    #[inline(always)]
    #[must_use]
    pub fn vbrs(&mut self) -> VBRS_W<9> {
        VBRS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWR Backup domain control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bdcr](index.html) module"]
pub struct BDCR_SPEC;
impl crate::RegisterSpec for BDCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bdcr::R](R) reader structure"]
impl crate::Readable for BDCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bdcr::W](W) writer structure"]
impl crate::Writable for BDCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BDCR to value 0"]
impl crate::Resettable for BDCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
