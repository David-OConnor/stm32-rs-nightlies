#[doc = "Register `APB1SECSR2` reader"]
pub struct R(crate::R<APB1SECSR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APB1SECSR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APB1SECSR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APB1SECSR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `LPUART1SECF` reader - LPUART1SECF"]
pub type LPUART1SECF_R = crate::BitReader<bool>;
#[doc = "Field `I2C4SECF` reader - I2C4SECF"]
pub type I2C4SECF_R = crate::BitReader<bool>;
#[doc = "Field `LPTIM2SECF` reader - LPTIM2SECF"]
pub type LPTIM2SECF_R = crate::BitReader<bool>;
#[doc = "Field `LPTIM3SECF` reader - LPTIM3SECF"]
pub type LPTIM3SECF_R = crate::BitReader<bool>;
#[doc = "Field `FDCAN1SECF` reader - FDCAN1SECF"]
pub type FDCAN1SECF_R = crate::BitReader<bool>;
#[doc = "Field `USBFSSECF` reader - USBFSSECF"]
pub type USBFSSECF_R = crate::BitReader<bool>;
#[doc = "Field `UCPD1SECF` reader - UCPD1SECF"]
pub type UCPD1SECF_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - LPUART1SECF"]
    #[inline(always)]
    pub fn lpuart1secf(&self) -> LPUART1SECF_R {
        LPUART1SECF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - I2C4SECF"]
    #[inline(always)]
    pub fn i2c4secf(&self) -> I2C4SECF_R {
        I2C4SECF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 5 - LPTIM2SECF"]
    #[inline(always)]
    pub fn lptim2secf(&self) -> LPTIM2SECF_R {
        LPTIM2SECF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - LPTIM3SECF"]
    #[inline(always)]
    pub fn lptim3secf(&self) -> LPTIM3SECF_R {
        LPTIM3SECF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 9 - FDCAN1SECF"]
    #[inline(always)]
    pub fn fdcan1secf(&self) -> FDCAN1SECF_R {
        FDCAN1SECF_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 21 - USBFSSECF"]
    #[inline(always)]
    pub fn usbfssecf(&self) -> USBFSSECF_R {
        USBFSSECF_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 23 - UCPD1SECF"]
    #[inline(always)]
    pub fn ucpd1secf(&self) -> UCPD1SECF_R {
        UCPD1SECF_R::new(((self.bits >> 23) & 1) != 0)
    }
}
#[doc = "RCC APB1 security status register 2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb1secsr2](index.html) module"]
pub struct APB1SECSR2_SPEC;
impl crate::RegisterSpec for APB1SECSR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [apb1secsr2::R](R) reader structure"]
impl crate::Readable for APB1SECSR2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets APB1SECSR2 to value 0"]
impl crate::Resettable for APB1SECSR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
