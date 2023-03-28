#[doc = "Register `IORETR` reader"]
pub struct R(crate::R<IORETR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IORETR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IORETR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IORETR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IORETR` writer"]
pub struct W(crate::W<IORETR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IORETR_SPEC>;
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
impl From<crate::W<IORETR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IORETR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IORETEN` reader - IO retention enable: When entering into standby mode, the output is sampled, and apply to the output IO during the standby power mode. Note: the IO state is not retained if the DBG_STANDBY bit is set in DBGMCU_CR register."]
pub type IORETEN_R = crate::BitReader<bool>;
#[doc = "Field `IORETEN` writer - IO retention enable: When entering into standby mode, the output is sampled, and apply to the output IO during the standby power mode. Note: the IO state is not retained if the DBG_STANDBY bit is set in DBGMCU_CR register."]
pub type IORETEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, IORETR_SPEC, bool, O>;
#[doc = "Field `JTAGIORETEN` reader - IO retention enable for JTAG IOs when entering into standby mode, the output is sampled, and apply to the output IO during the standby power mode"]
pub type JTAGIORETEN_R = crate::BitReader<bool>;
#[doc = "Field `JTAGIORETEN` writer - IO retention enable for JTAG IOs when entering into standby mode, the output is sampled, and apply to the output IO during the standby power mode"]
pub type JTAGIORETEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, IORETR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - IO retention enable: When entering into standby mode, the output is sampled, and apply to the output IO during the standby power mode. Note: the IO state is not retained if the DBG_STANDBY bit is set in DBGMCU_CR register."]
    #[inline(always)]
    pub fn ioreten(&self) -> IORETEN_R {
        IORETEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 16 - IO retention enable for JTAG IOs when entering into standby mode, the output is sampled, and apply to the output IO during the standby power mode"]
    #[inline(always)]
    pub fn jtagioreten(&self) -> JTAGIORETEN_R {
        JTAGIORETEN_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - IO retention enable: When entering into standby mode, the output is sampled, and apply to the output IO during the standby power mode. Note: the IO state is not retained if the DBG_STANDBY bit is set in DBGMCU_CR register."]
    #[inline(always)]
    #[must_use]
    pub fn ioreten(&mut self) -> IORETEN_W<0> {
        IORETEN_W::new(self)
    }
    #[doc = "Bit 16 - IO retention enable for JTAG IOs when entering into standby mode, the output is sampled, and apply to the output IO during the standby power mode"]
    #[inline(always)]
    #[must_use]
    pub fn jtagioreten(&mut self) -> JTAGIORETEN_W<16> {
        JTAGIORETEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWR I/O retention register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ioretr](index.html) module"]
pub struct IORETR_SPEC;
impl crate::RegisterSpec for IORETR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ioretr::R](R) reader structure"]
impl crate::Readable for IORETR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ioretr::W](W) writer structure"]
impl crate::Writable for IORETR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IORETR to value 0"]
impl crate::Resettable for IORETR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
