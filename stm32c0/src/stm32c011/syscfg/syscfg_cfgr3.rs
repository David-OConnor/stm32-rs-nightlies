#[doc = "Register `SYSCFG_CFGR3` reader"]
pub struct R(crate::R<SYSCFG_CFGR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYSCFG_CFGR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYSCFG_CFGR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYSCFG_CFGR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYSCFG_CFGR3` writer"]
pub struct W(crate::W<SYSCFG_CFGR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYSCFG_CFGR3_SPEC>;
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
impl From<crate::W<SYSCFG_CFGR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYSCFG_CFGR3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PINMUX0` reader - Pin GPIO multiplexer 0 This bit is set by software and cleared by system reset. It assigns a GPIO to a pin. 1x: Reserved Pin F2 of WLCSP14 package GPIO assignment 1x: Reserved"]
pub type PINMUX0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PINMUX0` writer - Pin GPIO multiplexer 0 This bit is set by software and cleared by system reset. It assigns a GPIO to a pin. 1x: Reserved Pin F2 of WLCSP14 package GPIO assignment 1x: Reserved"]
pub type PINMUX0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SYSCFG_CFGR3_SPEC, u8, u8, 2, O>;
#[doc = "Field `PINMUX1` reader - Pin GPIO multiplexer 1 This bit is set by software and cleared by system reset. It assigns a GPIO to a pin. 1x: Reserved"]
pub type PINMUX1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PINMUX1` writer - Pin GPIO multiplexer 1 This bit is set by software and cleared by system reset. It assigns a GPIO to a pin. 1x: Reserved"]
pub type PINMUX1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SYSCFG_CFGR3_SPEC, u8, u8, 2, O>;
#[doc = "Field `PINMUX2` reader - Pin GPIO multiplexer 2 This bit is set by software and cleared by system reset. It assigns a GPIO to a pin. 1x: Reserved 1x: Reserved"]
pub type PINMUX2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PINMUX2` writer - Pin GPIO multiplexer 2 This bit is set by software and cleared by system reset. It assigns a GPIO to a pin. 1x: Reserved 1x: Reserved"]
pub type PINMUX2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SYSCFG_CFGR3_SPEC, u8, u8, 2, O>;
#[doc = "Field `PINMUX3` reader - Pin GPIO multiplexer 3 This bit is set by software and cleared by system reset. It assigns a GPIO to a pin. 1x: Reserved"]
pub type PINMUX3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PINMUX3` writer - Pin GPIO multiplexer 3 This bit is set by software and cleared by system reset. It assigns a GPIO to a pin. 1x: Reserved"]
pub type PINMUX3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SYSCFG_CFGR3_SPEC, u8, u8, 2, O>;
#[doc = "Field `PINMUX4` reader - Pin GPIO multiplexer 4 This bit is set by software and cleared by system reset. It assigns a GPIO to a pin. 1x: Reserved 1x: Reserved"]
pub type PINMUX4_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PINMUX4` writer - Pin GPIO multiplexer 4 This bit is set by software and cleared by system reset. It assigns a GPIO to a pin. 1x: Reserved 1x: Reserved"]
pub type PINMUX4_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SYSCFG_CFGR3_SPEC, u8, u8, 2, O>;
#[doc = "Field `PINMUX5` reader - Pin GPIO multiplexer 5 This bit is set by software and cleared by system reset. It assigns a GPIO to a pin. 1x: Reserved"]
pub type PINMUX5_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PINMUX5` writer - Pin GPIO multiplexer 5 This bit is set by software and cleared by system reset. It assigns a GPIO to a pin. 1x: Reserved"]
pub type PINMUX5_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SYSCFG_CFGR3_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:1 - Pin GPIO multiplexer 0 This bit is set by software and cleared by system reset. It assigns a GPIO to a pin. 1x: Reserved Pin F2 of WLCSP14 package GPIO assignment 1x: Reserved"]
    #[inline(always)]
    pub fn pinmux0(&self) -> PINMUX0_R {
        PINMUX0_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Pin GPIO multiplexer 1 This bit is set by software and cleared by system reset. It assigns a GPIO to a pin. 1x: Reserved"]
    #[inline(always)]
    pub fn pinmux1(&self) -> PINMUX1_R {
        PINMUX1_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Pin GPIO multiplexer 2 This bit is set by software and cleared by system reset. It assigns a GPIO to a pin. 1x: Reserved 1x: Reserved"]
    #[inline(always)]
    pub fn pinmux2(&self) -> PINMUX2_R {
        PINMUX2_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Pin GPIO multiplexer 3 This bit is set by software and cleared by system reset. It assigns a GPIO to a pin. 1x: Reserved"]
    #[inline(always)]
    pub fn pinmux3(&self) -> PINMUX3_R {
        PINMUX3_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Pin GPIO multiplexer 4 This bit is set by software and cleared by system reset. It assigns a GPIO to a pin. 1x: Reserved 1x: Reserved"]
    #[inline(always)]
    pub fn pinmux4(&self) -> PINMUX4_R {
        PINMUX4_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Pin GPIO multiplexer 5 This bit is set by software and cleared by system reset. It assigns a GPIO to a pin. 1x: Reserved"]
    #[inline(always)]
    pub fn pinmux5(&self) -> PINMUX5_R {
        PINMUX5_R::new(((self.bits >> 10) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Pin GPIO multiplexer 0 This bit is set by software and cleared by system reset. It assigns a GPIO to a pin. 1x: Reserved Pin F2 of WLCSP14 package GPIO assignment 1x: Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn pinmux0(&mut self) -> PINMUX0_W<0> {
        PINMUX0_W::new(self)
    }
    #[doc = "Bits 2:3 - Pin GPIO multiplexer 1 This bit is set by software and cleared by system reset. It assigns a GPIO to a pin. 1x: Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn pinmux1(&mut self) -> PINMUX1_W<2> {
        PINMUX1_W::new(self)
    }
    #[doc = "Bits 4:5 - Pin GPIO multiplexer 2 This bit is set by software and cleared by system reset. It assigns a GPIO to a pin. 1x: Reserved 1x: Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn pinmux2(&mut self) -> PINMUX2_W<4> {
        PINMUX2_W::new(self)
    }
    #[doc = "Bits 6:7 - Pin GPIO multiplexer 3 This bit is set by software and cleared by system reset. It assigns a GPIO to a pin. 1x: Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn pinmux3(&mut self) -> PINMUX3_W<6> {
        PINMUX3_W::new(self)
    }
    #[doc = "Bits 8:9 - Pin GPIO multiplexer 4 This bit is set by software and cleared by system reset. It assigns a GPIO to a pin. 1x: Reserved 1x: Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn pinmux4(&mut self) -> PINMUX4_W<8> {
        PINMUX4_W::new(self)
    }
    #[doc = "Bits 10:11 - Pin GPIO multiplexer 5 This bit is set by software and cleared by system reset. It assigns a GPIO to a pin. 1x: Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn pinmux5(&mut self) -> PINMUX5_W<10> {
        PINMUX5_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SYSCFG configuration register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syscfg_cfgr3](index.html) module"]
pub struct SYSCFG_CFGR3_SPEC;
impl crate::RegisterSpec for SYSCFG_CFGR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [syscfg_cfgr3::R](R) reader structure"]
impl crate::Readable for SYSCFG_CFGR3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [syscfg_cfgr3::W](W) writer structure"]
impl crate::Writable for SYSCFG_CFGR3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SYSCFG_CFGR3 to value 0"]
impl crate::Resettable for SYSCFG_CFGR3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
