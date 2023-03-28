#[doc = "Register `SR` reader"]
pub struct R(crate::R<SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RTT4B` reader - FIFO is ready to transfer four bytes"]
pub type RTT4B_R = crate::BitReader<RTT4B_A>;
#[doc = "FIFO is ready to transfer four bytes\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RTT4B_A {
    #[doc = "0: FIFO is not ready for a four-byte transfer"]
    NotReady = 0,
    #[doc = "1: FIFO is ready for a four-byte (32-bit) transfer. In receive mode, this means that at least four valid data bytes are in the FIFO. In transmit mode, this means that there are at least four bytes free in the FIFO"]
    Ready = 1,
}
impl From<RTT4B_A> for bool {
    #[inline(always)]
    fn from(variant: RTT4B_A) -> Self {
        variant as u8 != 0
    }
}
impl RTT4B_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTT4B_A {
        match self.bits {
            false => RTT4B_A::NotReady,
            true => RTT4B_A::Ready,
        }
    }
    #[doc = "Checks if the value of the field is `NotReady`"]
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == RTT4B_A::NotReady
    }
    #[doc = "Checks if the value of the field is `Ready`"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == RTT4B_A::Ready
    }
}
#[doc = "Field `RTT1B` reader - FIFO is ready to transfer one byte"]
pub type RTT1B_R = crate::BitReader<RTT1B_A>;
#[doc = "FIFO is ready to transfer one byte\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RTT1B_A {
    #[doc = "0: FIFO is not ready for a 1-byte transfer"]
    NotReady = 0,
    #[doc = "1: FIFO is ready for a one byte (32-bit) transfer. In receive mode, this means that at least one valid data byte is in the FIFO. In transmit mode, this means that there is at least one byte free in the FIFO"]
    Ready = 1,
}
impl From<RTT1B_A> for bool {
    #[inline(always)]
    fn from(variant: RTT1B_A) -> Self {
        variant as u8 != 0
    }
}
impl RTT1B_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTT1B_A {
        match self.bits {
            false => RTT1B_A::NotReady,
            true => RTT1B_A::Ready,
        }
    }
    #[doc = "Checks if the value of the field is `NotReady`"]
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == RTT1B_A::NotReady
    }
    #[doc = "Checks if the value of the field is `Ready`"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == RTT1B_A::Ready
    }
}
impl R {
    #[doc = "Bit 2 - FIFO is ready to transfer four bytes"]
    #[inline(always)]
    pub fn rtt4b(&self) -> RTT4B_R {
        RTT4B_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - FIFO is ready to transfer one byte"]
    #[inline(always)]
    pub fn rtt1b(&self) -> RTT1B_R {
        RTT1B_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[doc = "PSSI status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr](index.html) module"]
pub struct SR_SPEC;
impl crate::RegisterSpec for SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sr::R](R) reader structure"]
impl crate::Readable for SR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SR to value 0"]
impl crate::Resettable for SR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
