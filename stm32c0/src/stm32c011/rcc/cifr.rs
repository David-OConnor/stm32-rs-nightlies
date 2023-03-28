#[doc = "Register `CIFR` reader"]
pub struct R(crate::R<CIFR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CIFR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CIFR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CIFR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `LSIRDYF` reader - LSI ready interrupt flag This flag indicates a pending interrupt upon LSE clock getting ready. Set by hardware when the LSI clock becomes stable and LSIRDYDIE is set. Cleared by software setting the LSIRDYC bit."]
pub type LSIRDYF_R = crate::BitReader<bool>;
#[doc = "Field `LSERDYF` reader - LSE ready interrupt flag This flag indicates a pending interrupt upon LSE clock getting ready. Set by hardware when the LSE clock becomes stable and LSERDYDIE is set. Cleared by software setting the LSERDYC bit."]
pub type LSERDYF_R = crate::BitReader<bool>;
#[doc = "Field `HSIRDYF` reader - HSI16 ready interrupt flag This flag indicates a pending interrupt upon HSI16 clock getting ready. Set by hardware when the HSI16 clock becomes stable and HSIRDYIE is set in response to setting the HSION (refer to ). When HSION is not set but the HSI16 oscillator is enabled by the peripheral through a clock request, this bit is not set and no interrupt is generated. Cleared by software setting the HSIRDYC bit."]
pub type HSIRDYF_R = crate::BitReader<bool>;
#[doc = "Field `HSERDYF` reader - HSE ready interrupt flag This flag indicates a pending interrupt upon HSE clock getting ready. Set by hardware when the HSE clock becomes stable and HSERDYIE is set. Cleared by software setting the HSERDYC bit."]
pub type HSERDYF_R = crate::BitReader<bool>;
#[doc = "Field `CSSF` reader - HSE clock security system interrupt flag This flag indicates a pending interrupt upon HSE clock failure. Set by hardware when a failure is detected in the HSE oscillator. Cleared by software setting the CSSC bit."]
pub type CSSF_R = crate::BitReader<bool>;
#[doc = "Field `LSECSSF` reader - LSE clock security system interrupt flag This flag indicates a pending interrupt upon LSE clock failure. Set by hardware when a failure is detected in the LSE oscillator. Cleared by software by setting the LSECSSC bit."]
pub type LSECSSF_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - LSI ready interrupt flag This flag indicates a pending interrupt upon LSE clock getting ready. Set by hardware when the LSI clock becomes stable and LSIRDYDIE is set. Cleared by software setting the LSIRDYC bit."]
    #[inline(always)]
    pub fn lsirdyf(&self) -> LSIRDYF_R {
        LSIRDYF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - LSE ready interrupt flag This flag indicates a pending interrupt upon LSE clock getting ready. Set by hardware when the LSE clock becomes stable and LSERDYDIE is set. Cleared by software setting the LSERDYC bit."]
    #[inline(always)]
    pub fn lserdyf(&self) -> LSERDYF_R {
        LSERDYF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - HSI16 ready interrupt flag This flag indicates a pending interrupt upon HSI16 clock getting ready. Set by hardware when the HSI16 clock becomes stable and HSIRDYIE is set in response to setting the HSION (refer to ). When HSION is not set but the HSI16 oscillator is enabled by the peripheral through a clock request, this bit is not set and no interrupt is generated. Cleared by software setting the HSIRDYC bit."]
    #[inline(always)]
    pub fn hsirdyf(&self) -> HSIRDYF_R {
        HSIRDYF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - HSE ready interrupt flag This flag indicates a pending interrupt upon HSE clock getting ready. Set by hardware when the HSE clock becomes stable and HSERDYIE is set. Cleared by software setting the HSERDYC bit."]
    #[inline(always)]
    pub fn hserdyf(&self) -> HSERDYF_R {
        HSERDYF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - HSE clock security system interrupt flag This flag indicates a pending interrupt upon HSE clock failure. Set by hardware when a failure is detected in the HSE oscillator. Cleared by software setting the CSSC bit."]
    #[inline(always)]
    pub fn cssf(&self) -> CSSF_R {
        CSSF_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - LSE clock security system interrupt flag This flag indicates a pending interrupt upon LSE clock failure. Set by hardware when a failure is detected in the LSE oscillator. Cleared by software by setting the LSECSSC bit."]
    #[inline(always)]
    pub fn lsecssf(&self) -> LSECSSF_R {
        LSECSSF_R::new(((self.bits >> 9) & 1) != 0)
    }
}
#[doc = "RCC clock interrupt flag register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cifr](index.html) module"]
pub struct CIFR_SPEC;
impl crate::RegisterSpec for CIFR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cifr::R](R) reader structure"]
impl crate::Readable for CIFR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CIFR to value 0"]
impl crate::Resettable for CIFR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
