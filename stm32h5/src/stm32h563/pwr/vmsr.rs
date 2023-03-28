#[doc = "Register `VMSR` reader"]
pub struct R(crate::R<VMSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VMSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VMSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VMSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `AVDO` reader - analog voltage detector output on V<sub>DDA</sub> This bit is set and cleared by hardware. It is valid only if AVD on VDDA is enabled by the AVDEN bit. Note: Since the AVD is disabled in Standby mode, this bit is equal to 0 after standby or reset until the AVDEN bit is set."]
pub type AVDO_R = crate::BitReader<bool>;
#[doc = "Field `VDDIO2RDY` reader - voltage detector output on V<sub>DDIO2</sub> This bit is set and cleared by hardware."]
pub type VDDIO2RDY_R = crate::BitReader<bool>;
#[doc = "Field `PVDO` reader - programmable voltage detect output This bit is set and cleared by hardware. It is valid only if the PVD has been enabled by the PVDE bit. Note: Since the PVD is disabled in Standby mode, this bit is equal to 0 after Standby or reset until the PVDE bit is set."]
pub type PVDO_R = crate::BitReader<bool>;
#[doc = "Field `USB33RDY` reader - V<sub>DDUSB</sub> ready"]
pub type USB33RDY_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 19 - analog voltage detector output on V<sub>DDA</sub> This bit is set and cleared by hardware. It is valid only if AVD on VDDA is enabled by the AVDEN bit. Note: Since the AVD is disabled in Standby mode, this bit is equal to 0 after standby or reset until the AVDEN bit is set."]
    #[inline(always)]
    pub fn avdo(&self) -> AVDO_R {
        AVDO_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - voltage detector output on V<sub>DDIO2</sub> This bit is set and cleared by hardware."]
    #[inline(always)]
    pub fn vddio2rdy(&self) -> VDDIO2RDY_R {
        VDDIO2RDY_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 22 - programmable voltage detect output This bit is set and cleared by hardware. It is valid only if the PVD has been enabled by the PVDE bit. Note: Since the PVD is disabled in Standby mode, this bit is equal to 0 after Standby or reset until the PVDE bit is set."]
    #[inline(always)]
    pub fn pvdo(&self) -> PVDO_R {
        PVDO_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 24 - V<sub>DDUSB</sub> ready"]
    #[inline(always)]
    pub fn usb33rdy(&self) -> USB33RDY_R {
        USB33RDY_R::new(((self.bits >> 24) & 1) != 0)
    }
}
#[doc = "PWR voltage monitor status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vmsr](index.html) module"]
pub struct VMSR_SPEC;
impl crate::RegisterSpec for VMSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [vmsr::R](R) reader structure"]
impl crate::Readable for VMSR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets VMSR to value 0"]
impl crate::Resettable for VMSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
